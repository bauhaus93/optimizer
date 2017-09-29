
use yup_oauth2 as oauth2;
use yup_oauth2:: {  Authenticator, DefaultAuthenticatorDelegate, PollInformation,
                    ConsoleApplicationSecret, MemoryStorage, GetToken };
use serde_json as json;

pub struct Connection {


}

impl Connection {


    pub fn new() -> Result<Connection, ()> {

        match json::from_str::<ConsoleApplicationSecret>("bla") {
            Ok(e) => {},
            Err(e) => {
                error!("Could not read secret: {}", e);
                return Err(());
            }
        }

        let connection = Connection {

        };

        Ok(connection)
    }



}
