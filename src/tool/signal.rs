use super::part::Part;

pub enum Signal {
    Start,
    Stop,
    LoadPart(Part),
    UnloadPart,
    ProcessFinished
}