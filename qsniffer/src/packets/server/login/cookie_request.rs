#[qexed_packet_macros::packet(id = 0x05)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CookieRequest {
}
impl CookieRequest {
    pub fn new() -> Self {
        CookieRequest {
        }
    }
}
