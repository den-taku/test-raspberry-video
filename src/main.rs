//! from [here](https://github.com/Pragmatic-Elixir-Meetup/rpi-video-rs/blob/master/examples/simple.rs)
use rpi_video_rs::recorder::Recorder;
use rpi_video_rs::video_param::VideoParam;

fn main() {
    println!("\nStart to record a new H264 video\n");

    let config = VideoParam {
        output_file_path: "video.h264",
        ..VideoParam::default()
    };

    let mut recorder = Recorder::new(config);

    match recorder.run() {
        Ok(res) => println!(
            "A new H264 video is generated to `{}`\n",
            res.output_file_path
        ),
        Err(err) => println!("An error occurred - `{}`\n", err.message),
    }

    println!("\nFinish recording\n");
}
