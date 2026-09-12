use std::env;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler{
    async fn message(&self, ctx:Context, msg:Message){

    }
}
