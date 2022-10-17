use graphic_offsets::GraphicOffsetsError;
use inject_helper::InjectHelperError;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum ObsError {
    ProcessNotFound,
    Inject(InjectHelperError),
    LoadGraphicOffsets(GraphicOffsetsError),
    CreatePipe,
    CreateMutex,
    CreateEvent,
    CreateFileMapping(u32),
    CreateDevice,
    OpenSharedResource,
    CreateTexture,
    MapSurface,
}

impl Display for ObsError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ObsError::ProcessNotFound => write!(f, "ProcessNotFound"),
            ObsError::Inject(e) => write!(f, "Inject: {:?}", e),
            ObsError::LoadGraphicOffsets(e) => write!(f, "LoadGraphicOffsets: {:?}", e),
            ObsError::CreatePipe => write!(f, "CreatePipe"),
            ObsError::CreateMutex => write!(f, "CreateMutex"),
            ObsError::CreateEvent => write!(f, "CreateEvent"),
            ObsError::CreateFileMapping(n) => write!(f, "CreateFileMapping: {:?}", n),
            ObsError::CreateDevice => write!(f, "CreateDevice"),
            ObsError::OpenSharedResource => write!(f, "OpenSharedResource"),
            ObsError::CreateTexture => write!(f, "CreateTexture"),
            ObsError::MapSurface => write!(f, "MapSurface"),
        };

        Ok(())
    }
}
