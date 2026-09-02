use tracing::debug;

use crate::data::{
    Data,
    Error,
};

pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>)
{
    match error
    {
        poise::FrameworkError::Command { error, ctx, .. } =>
        {
            debug!("Error in command `{}`: {:?}", ctx.command().name, error);
        },

        error =>
        {
            if let Err(e) = poise::builtins::on_error(error).await
            {
                debug!("Error while handling error: {}", e);
            }
        },
    }
}
