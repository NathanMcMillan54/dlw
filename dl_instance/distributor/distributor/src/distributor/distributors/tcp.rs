use lib_dldistributor::external::{ExternalDistributorInfo, ExternalDistributorRW};
use std::net::TcpStream;

struct TcpDistributor {
    pub stream: TcpStream,
    pub info: ExternalDistributorInfo,
}

impl TcpDistributor {
    pub fn new_and_check(stream: TcpStream) -> Option<Self> {
        let mut tcp_dist = TcpDistributor {
            stream: stream,
            info: ExternalDistributorInfo::default(),
        };

        return if tcp_dist.verify_distributor() {
            Some(tcp_dist)
        } else {
            None
        };
    }
}

impl ExternalDistributorRW for TcpDistributor {
    fn verify_distributor(&mut self) -> bool {
        false
    }
}
