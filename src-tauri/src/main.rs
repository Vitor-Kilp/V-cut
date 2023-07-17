// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::process::{Command, CommandEvent};
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_file_in_default_program(file_path: &str) -> Result<(), String> {
    let cmd = Command::new("powershell.exe")
        .args([
            "-Command".to_string(),
            format!("Start-Process -FilePath \"{file_path}\""),
        ])
        .spawn();

    match cmd {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Failed to execute PowerShell command: {}", err)),
    }
}

#[tauri::command]
fn copy_file_to_clipboard(file_path: String) -> Result<String, String> {
    let cmd = Command::new("powershell.exe")
        .args([
            "-Command".to_string(),
            format!(
                "Set-Clipboard -Path \"{}\"",
                file_path.replace("\\", "\\\\")
            ),
        ])
        .spawn();

    match cmd {
        Ok(_) => Ok("OK".to_string()),
        Err(err) => Err(format!("Failed to execute PowerShell command: {}", err)),
    }
}

#[tauri::command]
async fn transcode_video_cpu(
    window: Window,
    input_file: &str,
    output_file: &str,
    crf: &str,
    start_time: &str,
    end_time: &str,
    encoding: &str,
    resolution: &str
) -> Result<String, tauri::Error> {

    let resolution_to_use = format!("scale=-1:{resolution}");

    let arguments = [
        "-progress",
        "pipe:1",
        "-i",
        input_file,
        "-ss",
        start_time,
        "-to",
        end_time,
        "-c:v",
        encoding,
        "-crf",
        crf,
        "-vf",
        &resolution_to_use,
        output_file,
    ];
    let ffmpeg_command = Command::new("ffmpeg").args(arguments).spawn();

    match ffmpeg_command {
        Ok((mut rx, mut _child)) => {
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    if line.contains("out_time=") {
                        let progress = line.split("=").last().unwrap();
                        window.emit("progress", progress).unwrap();
                    }
                    //println!("got: {}", line);
                }
            }
            return Ok(format!("OK"));
        }
        Err(err) => {
            //println!("{err:?}");
            return Err(tauri::Error::FailedToExecuteApi(err))
        },
    }
}

#[tauri::command]
async fn transcode_video_gpu(
    window: Window,
    input_file: &str,
    output_file: &str,
    bitrate: &str,
    start_time: &str,
    end_time: &str,
    encoding: &str,
    resolution: &str
) -> Result<String, tauri::Error> {
    // Try enconding one frame to test for Nvenc
    let test_for_nvenc = Command::new("ffmpeg").args("ffmpeg -loglevel error -f lavfi -i color=black:s=1080x1080 -vframes 1 -an -c:v h264_nvenc -f null -".split(" ")).spawn();

    // Use amd encoding otherwise
    let encoding_to_use = match test_for_nvenc {
        Ok(_) => {
            if encoding == "libx264" {
                "h264_nvenc"
            } else {
                "hevc_nvenc"
            }
        }
        Err(_) => {
            if encoding == "libx264" {
                "h264_amf"
            } else {
                "hevc_amf"
            }
        }
    };

    let resolution_to_use = format!("scale=-1:{resolution}");

    let arguments = [
        "-progress",
        "pipe:1",
        "-i",
        input_file,
        "-ss",
        start_time,
        "-to",
        end_time,
        "-c:v",
        encoding_to_use,
        "-b:v",
        bitrate,
        "-vf",
        &resolution_to_use,
        output_file,
    ];

    let ffmpeg_command = Command::new("ffmpeg").args(arguments).spawn();

    match ffmpeg_command {
        Ok((mut rx, mut _child)) => {
            while let Some(event) = rx.recv().await {
                if let CommandEvent::Stdout(line) = event {
                    if line.contains("out_time=") {
                        let progress = line.split("=").last().unwrap();
                        window.emit("progress", progress).unwrap();
                    }
                    //println!("got: {}", line);
                }
            }
            return Ok(format!("OK"));
        }
        Err(err) => return Err(tauri::Error::FailedToExecuteApi(err)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            transcode_video_cpu,
            transcode_video_gpu,
            open_file_in_default_program,
            copy_file_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
