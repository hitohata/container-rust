use crate::cli::Args;
use crate::config::ContainerOps;
use crate::errors::ErrCode;

pub struct Container {
    config: ContainerOps,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, ErrCode> {
        let config = ContainerOps::new(args.command, args.uid, args.mount_dir)?;

        Ok(Container { config })
    }

    pub fn crete(&mut self) -> Result<(), ErrCode> {
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), ErrCode> {
        log::debug!("Cleaning container");
        Ok(())
    }

    pub fn start(args: Args) -> Result<(), ErrCode> {
        let mut container = Container::new(args)?;
        if let Err(e) = container.crete() {
            container.clean_exit()?;
            log::error!("Error while creating container: {}", e);
            return Err(e);
        }
        log::debug!("Finished cleaing & exit");
        container.clean_exit()
    }
}
