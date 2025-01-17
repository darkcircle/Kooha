use pulsectl::{controllers::types::ServerInfo, ControllerError};

fn retrieve_server_info() -> Result<ServerInfo, ControllerError> {
    let mut source_controller = pulsectl::controllers::SourceController::create()?;
    let server_info = source_controller.get_server_info()?;
    Ok(server_info)
}

pub fn server_version_info() -> Option<String> {
    let server_info = retrieve_server_info().ok()?;
    let server_name = server_info.server_name?;
    let server_version = server_info.server_version?;
    Some(format!("{} version {}", server_name, server_version))
}

pub fn default_audio_devices_name() -> (Option<String>, Option<String>) {
    let server_info = match retrieve_server_info() {
        Ok(server_info) => server_info,
        Err(error) => {
            log::error!("Failed to retrieve server info: {:?}", error);
            return (None, None);
        }
    };

    let default_sink_name = server_info
        .default_sink_name
        .map(|name| format!("{}.monitor", name));
    let default_source_name = server_info.default_source_name;

    if default_sink_name == default_source_name {
        (default_sink_name, None)
    } else {
        (default_sink_name, default_source_name)
    }
}
