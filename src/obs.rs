use obws::Client;


struct Obs{
    client: Option<Client>,
}

impl Obs{
    pub async fn connect(&mut self,host:&str,port: u16,password: Option<&str>) -> Result<(),obws::error::Error>{
        let client = Client::connect(host, port, password).await?;
        self.client = Some(client);
        Ok(())
    }

    async fn get_replay_buffer_status(&self,client:&Client) -> Result<bool,String>{
        let res = client.replay_buffer().status().await;
        match res{
            Ok(res) => Ok(res),
            Err(_) => Err("failed to get replay_buffer status".to_string())
        }
    }
    pub async fn set_replay_buffer(&self) -> Result<(), String>{
        let client = &self.client;
        let client = match client{
            Some(c) => c,
            None => return Err("failed to get client".to_string())
        };
        let status = self.get_replay_buffer_status(client).await?;
        if status {
            return Ok(());
        }

        Ok(())
    }
}

