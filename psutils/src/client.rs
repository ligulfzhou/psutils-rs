use crate::errors::{PSError, PSResult};
use ssh2::Session;
use std::fs::File;
use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;

pub struct Client {
    username: String,
    password: Option<String>,
    private_key_path: Option<String>,
    private_key_string: Option<String>,
    host: String,
    port: Option<u32>,
    sess: Option<Arc<Session>>,
}

impl Client {
    pub fn new(
        host: String,
        port: Option<u32>,
        username: String,
        password: Option<String>,
        private_key_path: Option<String>,
        private_key_string: Option<String>,
    ) -> Client {
        Client {
            username,
            password,
            private_key_path,
            private_key_string,
            host,
            port,
            sess: None,
        }
    }

    pub fn connect(&mut self) -> PSResult<()> {
        let tcp = TcpStream::connect(format!("{}:{}", &self.host, self.port.unwrap_or(22)))
            .map_err(|_err| PSError::SessionError("TCP Stream failed".to_string()))?;

        let mut sess = Session::new()
            .map_err(|_err| PSError::SessionError("New Session failed".to_string()))?;
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();

        let password: &str = {
            if let Some(pass) = &self.password {
                pass
            } else {
                ""
            }
        };
        let mut content = String::new();
        let mut private_key_string: &str = {
            if let Some(private_key_path) = &self.private_key_path {
                let mut file = File::open(private_key_path).map_err(|err| PSError::IOError(err))?;
                let _size = file
                    .read_to_string(&mut content)
                    .map_err(|err| PSError::IOError(err))?;
                &content
            } else if let Some(private_key_string) = &self.private_key_string {
                private_key_string
            } else {
                ""
            }
        };

        if password != "" && private_key_string != "" {
            // private key + keypass
            sess.userauth_pubkey_memory(&self.username, None, private_key_string, Some(password))
                .map_err(|err| {
                    PSError::ConnectinError(
                        "userauth with private key+passphrase failed.".to_string(),
                    )
                })?;
        } else if password == "" && private_key_string != "" {
            // private key
            sess.userauth_pubkey_memory(&self.username, None, private_key_string, None)
                .map_err(|err| {
                    PSError::ConnectinError("userauth with private key failed.".to_string())
                })?;
        } else if password != "" && private_key_string == "" {
            // password
            sess.userauth_password(&self.username, password)
                .map_err(|err| PSError::ConnectinError("userauth_password failed".to_string()))?;
        } else {
            // won't happen...
        }

        self.sess = Some(Arc::new(sess));

        Ok(())
    }

    pub fn exists(&self, file: &str) -> PSResult<bool> {
        todo!()
    }

    pub fn content(&mut self, file: &str) -> PSResult<String> {
        let res = self.exec(&format!("cat {}", file))?;
        // let ss: Vec<&str> = res.split('\n').collect();
        Ok(res)
    }

    pub fn exec(&self, cmd: &str) -> PSResult<String> {
        match &self.sess {
            Some(s) => {
                let cli = s.clone();
                let mut channel = cli.channel_session().unwrap();
                channel.exec(cmd).unwrap();
                let mut s = String::new();
                channel.read_to_string(&mut s).unwrap();
                println!("{}", s);
                return Ok(s);
            }
            _ => {
                // let _ = self.connect();
                return Err(PSError::ConnectinError("Not Connected...".to_string()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::client::Client;

    #[test]
    fn test_connection() {
        let mut client = Client::new(
            "your_ip".to_string(),
            Some(22),
            "your_user".to_string(),
            Some("your_password".to_string()),
            None,
            None,
        );
        client.connect();
        let res = client.exec("uptime");
    }
}
