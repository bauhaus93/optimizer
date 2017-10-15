
use hyper;
use hyper::{ Client, Method, Request };
use hyper::header;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::Future;


use client::connection_error::ConnectionError;
use client::token;
use client::authorization_header;

pub struct Connection {
}


impl Connection {

    pub fn new(token_path: &str) -> Result<Connection, ConnectionError> {

        let realm = "http://www.mkmapi.eu/ws/v1.1/account";

        info!("retrieving app token from file \"{}\"", token_path);
        let token = try!(token::parse_app_token(token_path));
        info!("parsed token: {},", token);

        let auth_hdr = try!(authorization_header::AuthorizationHeader::new(
            token.clone(),
            realm
        ));

        //info!("authorization header: {}", auth_hdr);

        let uri = try!(realm.parse::<hyper::Uri>());

        let mut request: Request = Request::new(Method::Get, uri);
        request.headers_mut().set(header::Authorization(auth_hdr));

        let mut core = try!(Core::new());
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &core.handle()).unwrap()) //TODO handle error case native_tls
            .build(&core.handle());

        let work = client.request(request).and_then(| res | {
            info!("response: {}", res.status());
            Ok(())
        });

        try!(core.run(work));

        Ok(Connection {})
    }


}
