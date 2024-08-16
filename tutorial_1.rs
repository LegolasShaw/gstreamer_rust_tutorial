use gstreamer::{init};
use gstreamer::ffi::{gst_parse_launch, gst_element_set_state, gst_element_get_bus, gst_bus_timed_pop_filtered,
                     gst_element_factory_make,
                     GST_CLOCK_TIME_NONE, GST_MESSAGE_ERROR, GST_MESSAGE_EOS, GST_STATE_PLAYING};
use gstreamer::glib::translate::ToGlibPtr;

fn main() {
    _ = init();
    let mut error = std::ptr::null_mut();
    unsafe {
        let pipeline_description = "playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm";
        let pipeline = gst_parse_launch(pipeline_description.to_glib_none().0, &mut error);
        let _ = gst_element_set_state(pipeline, GST_STATE_PLAYING);
        let bus = gst_element_get_bus(pipeline);
        let msg = gst_bus_timed_pop_filtered(bus, GST_CLOCK_TIME_NONE, GST_MESSAGE_ERROR|GST_MESSAGE_EOS);
        if (*msg).type_ == GST_MESSAGE_ERROR {
            println!("{:?}", "An error occurred! Re-run with the GST_DEBUG=*:WARN  environment variable set for more details.\n");
        }
    };
}
