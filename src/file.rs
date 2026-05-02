use tokio::fs;
use tokio::io::AsyncWriteExt;
use std::path::Path;

pub struct File {
    path: String,
}

pub enum WriteMode {
    Overwrite,
    Append,
}

impl File {

    pub fn open(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub async fn put(
        &self,
        data: impl AsRef<[u8]>
    ) -> Result<&Self, Box<dyn std::error::Error>> {

        fs::write(&self.path, data).await?;
        Ok(self)
    }

    pub async fn append(
        &self,
        data: impl AsRef<[u8]>
    ) -> Result<&Self, Box<dyn std::error::Error>> {

        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await?;

        file.write_all(data.as_ref()).await?;
        Ok(self)
    }

    pub async fn put_mode(
        &self,
        data: impl AsRef<[u8]>,
        mode: WriteMode
    ) -> Result<&Self, Box<dyn std::error::Error>> {

        match mode {
            WriteMode::Overwrite => {
                fs::write(&self.path, data).await?;
            }
            WriteMode::Append => {
                let mut file = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&self.path)
                    .await?;

                file.write_all(data.as_ref()).await?;
            }
        }

        Ok(self)
    }

    pub async fn read(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(&self.path).await?)
    }

    pub async fn bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(fs::read(&self.path).await?)
    }

    pub fn exists(&self) -> bool {
        Path::new(&self.path).exists()
    }

    pub async fn delete(&self) -> Result<bool, Box<dyn std::error::Error>> {
        fs::remove_file(&self.path).await?;
        Ok(true)
    }

    pub async fn copy(&self, to: &str) -> Result<bool, Box<dyn std::error::Error>> {
        fs::copy(&self.path, to).await?;
        Ok(true)
    }

    pub async fn rename(&self, new: &str) -> Result<bool, Box<dyn std::error::Error>> {
        fs::rename(&self.path, new).await?;
        Ok(true)
    }

    pub fn is_file(&self) -> bool {
        Path::new(&self.path).is_file()
    }

    pub fn is_dir(&self) -> bool {
        Path::new(&self.path).is_dir()
    }

    pub async fn mkdir(&self) -> Result<bool, Box<dyn std::error::Error>> {
        fs::create_dir(&self.path).await?;
        Ok(true)
    }

    pub async fn mkdir_all(&self) -> Result<bool, Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.path).await?;
        Ok(true)
    }

    pub async fn rmdir(&self) -> Result<bool, Box<dyn std::error::Error>> {
        fs::remove_dir(&self.path).await?;
        Ok(true)
    }

    pub async fn rmdir_all(&self) -> Result<bool, Box<dyn std::error::Error>> {
        fs::remove_dir_all(&self.path).await?;
        Ok(true)
    }

    pub async fn scandir(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();

        let mut entries = fs::read_dir(&self.path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            files.push(name);
        }

        Ok(files)
    }
    
    pub async fn unlink(&self) -> bool {
        fs::remove_file(&self.path).await.is_ok()
    }

    pub async fn mv(&self, to: &str) -> Result<bool, Box<dyn std::error::Error>> {

        if fs::rename(&self.path, to).await.is_ok() {
            return Ok(true);
        }

        fs::copy(&self.path, to).await?;
        fs::remove_file(&self.path).await?;

        Ok(true)
    }
}