
use gstreamer::ffi::{GstElement, GstPad, GstBin, gst_element_get_static_pad, gst_caps_get_structure,
                     gst_pad_is_linked, gst_pad_link, gst_pad_get_current_caps, gst_structure_get_name,
                     gst_element_factory_make, gst_pipeline_new, gst_bin_add_many, GST_PAD_LINK_OK,
                     gst_element_link_many};
use std::{ffi::CStr, ffi::CString, future::Future, mem, num::NonZeroU64, pin::Pin};
use std::ffi::c_char;
use gstreamer::glib::ffi::{g_str_has_prefix, GTRUE};
use gstreamer::glib::gobject_ffi::{g_object_set_data, g_signal_connect_data, GObject, GCallback};
use gstreamer::glib::translate::ToGlibPtr;
use gstreamer::init;
use gtk::glib::ffi::gpointer;
/* Structure to contain all our information, so we can pass it to callbacks */
#[derive(Debug)]
pub struct CustomData {
    pub pipeline: *mut GstElement,
    pub source: *mut GstElement,
    pub convert: *mut GstElement,
    pub resample: *mut GstElement,
    pub sink: *mut GstElement
}

/* This function will be called by the pad-added signal */
unsafe fn  pad_added_handler(src: *mut GstElement, new_pad: *mut GstPad, data: CustomData) {
    let sink_pad = gst_element_get_static_pad(data.convert, CString::new("sink".to_owned()).unwrap().as_c_str().as_ptr());
    let src_name = CStr::from_ptr((*src).object.name).to_str().unwrap();
    let new_pad_name = CStr::from_ptr((*new_pad).object.name).to_str().unwrap();

    println!("Received new pad {:?} from {:?}:\n", new_pad_name, src_name);


    if gst_pad_is_linked (sink_pad) == GTRUE {
        println! ("We are already linked. Ignoring.\n");
        return ;
    }

    let new_pad_caps = gst_pad_get_current_caps(new_pad);
    let new_pad_struct = gst_caps_get_structure(new_pad_caps, 0);
    let new_pad_type = gst_structure_get_name(new_pad_struct);

    let prefix = CString::new("audio/x-raw".to_owned()).unwrap().as_c_str().as_ptr();
    if g_str_has_prefix(new_pad_type, prefix) == GTRUE {
        return ;
    }

    let ret = gst_pad_link(new_pad, sink_pad);
    if ret != GST_PAD_LINK_OK  {
        println! ("Type is {:?} but link failed.\n", CStr::from_ptr(new_pad_type).to_str());
    } else {
        println! ("Link succeeded (type {:?}).\n", CStr::from_ptr(new_pad_type).to_str());
    }
}

unsafe  fn rust_str_to_c_str(str: &str) -> *const c_char {
    CString::new(str.to_owned()).unwrap().as_c_str().as_ptr()
}
fn main() {
    _ = init();
    let test = unsafe { rust_str_to_c_str("audioconvert") };
    let test1: *const c_char = "audioconvert".to_glib_none().0;
    let source = unsafe {gst_element_factory_make("uridecodebin".to_glib_none().0, "source".to_glib_none().0)};
    let convert = unsafe {gst_element_factory_make("audioconvert".to_glib_none().0, "convert".to_glib_none().0)};

    let resample  = unsafe {gst_element_factory_make("audioresample".to_glib_none().0, "resample".to_glib_none().0)};
    let sink   = unsafe {gst_element_factory_make("autoaudiosink".to_glib_none().0, "sink".to_glib_none().0)};

    let pipeline = unsafe { gst_pipeline_new("test-pipeline".to_glib_none().0) };
    let data = CustomData{
        pipeline,
        source,
        convert,
        resample,
        sink,
    };


    let source_ref = &data.source;
    if data.pipeline.is_null() || data.source.is_null() || data.convert.is_null() || data.resample.is_null() || data.sink.is_null(){
        println! ("Not all elements could be created.\n");
        return ;
    }
    let null_ptr: *const u32 = std::ptr::null();
    unsafe { gst_bin_add_many(data.pipeline as *mut GstBin, *source_ref, data.convert, data.resample, data.sink, null_ptr); }

    unsafe {
        if gst_element_link_many(data.convert, data.resample, &data, sink, null_ptr) != GTRUE {
            println!("Elements could not be linked.\n");
            return ;
        }
    }

    println!("{:?}", data);

    let uri_key = "uri";


    let c_str = CString::new("https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm".to_owned()).unwrap();
    let c_str_ptr = c_str.as_c_str().as_ptr();
    unsafe { g_object_set_data(*source_ref as *mut GObject, uri_key.to_glib_none().0,
                               c_str_ptr as gpointer); }

    // let handler : GCallback = unsafe { std::mem::transmute(pad_added_handler) };

    let raw_ptr: *mut CustomData = Box::into_raw(Box::new(data));
    println!("{:?}", raw_ptr);
    // unsafe { g_signal_connect_data(*source_ref as *mut GObject, "pad-added".to_glib_none().0, handler, raw_ptr as gpointer) }
}

