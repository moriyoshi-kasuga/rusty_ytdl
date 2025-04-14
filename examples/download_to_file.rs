use rusty_ytdl::*;

#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=FZ8BxMU3BYc";

    let video = Video::new_with_options(
        url,
        VideoOptions {
            quality: VideoQuality::HighestVideo,
            filter: VideoSearchOptions::Video,
            ..Default::default()
        },
    )
    .unwrap();

    let info = video.get_info().await.unwrap();

    println!("Video Title: {:#?}", info.video_details.title);
    println!("Video Length: {:#?}", info.video_details.length_seconds);

    let format = video.choose_format(&info.formats).unwrap();

    let path = std::path::Path::new(r"test.mp4");

    format
        .download(video.get_client(), video.get_options(), path)
        .await
        .unwrap();
}
