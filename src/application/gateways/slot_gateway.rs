pub trait SlotGateway {
    fn get_latest(&self) -> Result<u64, String>;
}
