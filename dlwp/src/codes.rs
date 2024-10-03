#[derive(Debug, PartialOrd, PartialEq)]
#[repr(u16)]
pub enum Code {
    /// ``Response`` (code >= 100) is used for server and client communication
    Request(u16),
    /// ``Response`` (code >= 200) is used for server and client communication
    Response(u16),
    /// ``Error`` (code >= 300) is intended for returning an error from an operation from Darklight that isn't related
    /// directly to a server or client
    Error(u16),
    /// ``Status`` (code <= 400) is intended for returning the status of an operation from Darklight that isn't
    /// related directly to a server or client
    Status(u16),
}

impl Code {
    #[allow(unreachable_code)]
    pub const fn new(value: u16) -> Self {
        return if value < 200 {
            return Code::Request(value);
        } else if value < 300 {
            return Code::Response(value);
        } else if value < 400 {
            return Code::Error(value);
        } else {
            return Code::Status(value);
        };
    }

    pub fn value(self) -> u16 {
        return match self {
            Code::Request(req) => req,
            Code::Response(rsp) => rsp,
            Code::Error(err) => err,
            Code::Status(stat) => stat,
        };
    }
}

/// Requests a connection from the Server or Client
pub const REQUEST_CONNECTION: Code = Code::Request(0);
/// Requests that the server or client responds
pub const REQUEST_RESPONSE: Code = Code::Request(1);
/// Request a chunk of a large piece of data, usually comes after specifying the type of data
pub const REQUEST_CHUNK: Code = Code::Request(2);
/// Requests a file, if the file can fit into a ``Message`` it should be sent in the next response, if not it should be
/// sent in chunks
pub const REQUEST_FILE: Code = Code::Request(3);
/// Requests if changing Server/Client ports will be acceptable
pub const REQUEST_PORT_SWITCH: Code = Code::Request(4);

/// Response for a connection being accepted, should be sent after requesting a connection
pub const CONNECTION_ACCEPTED: Code = Code::Response(200);
/// Response for when a connection is being denied
pub const CONNECTION_DENIED: Code = Code::Response(201);
/// Response for any type of request, should be sent after requesting a "regular" response
pub const REGULAR_RESPONSE: Code = Code::Response(202);
/// Responed with a chunk of a large piece of data, should coem after a chunk has been requested
pub const CHUNK_RESPONSE: Code = Code::Response(203);
/// Response for a file that fits inside a ``Message`` or wether the file can be sent or not. If the file needs to be
/// sent in chunks respond with the number of chunks that will need to be received
pub const FILE_RESPONSE: Code = Code::Response(204);
/// Disconnect server/client
pub const DISCONNECT: Code = Code::Response(205);
/// When a client requests or responds with something unexpected
pub const INVALID_RR: Code = Code::Response(206);

/// Internal error indicating that a Message will be longer than 4096 bytes
pub const LENGTH_EXCEEDED: Code = Code::Error(300);
/// Internal error indicating that the Distributor Id is not valid
pub const INVALID_DID: Code = Code::Error(301);
/// Internal error indicating that the distributor could not be found
pub const DISTRIBUTOR_NOT_FOUND: Code = Code::Error(302);
/// Internal error indicating message is incomplete (missing identifiers)
pub const MESSAGE_INCOMPLETE: Code = Code::Error(303);
/// Internal error indicating that a stream file is either missing or could not be found
pub const STREAM_FILE_NOT_FOUND: Code = Code::Error(304);

pub const READ_TIMEDOUT: Code = Code::Status(400);
pub const WRITE_TIMEDOUT: Code = Code::Status(401);
pub const READ_SUCCESS: Code = Code::Status(402);
pub const WRITE_SUCCESS: Code = Code::Status(403);
pub const UNKNOWN_STATUS: Code = Code::Status(404);
pub const STATUS_OK: Code = Code::Status(405);
pub const WRITE_FAILED: Code = Code::Status(406);
pub const READ_FAILED: Code = Code::Status(407);
pub const REMOVE_CLIENT: Code = Code::Status(408);
/// Do what you want with this
pub const TEAPOT: Code = Code::Status(418);
