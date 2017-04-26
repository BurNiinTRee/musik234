extern crate regex;
extern crate discord;
#[macro_use]
extern crate error_chain;

mod error;


use error::*;

use discord::Discord;
use discord::model::ChannelType;

fn run() -> Result<()> {
    let discord = Discord::from_bot_token(include_str!("../token"))?;
    let server_id = discord.get_servers()?[0].id;
    let channels = discord.get_server_channels(server_id)?;

    let voice_channels: Vec<_> = channels.iter()
        .filter(|x| if let ChannelType::Voice = x.kind {
                    true
                } else {
                    false
                })
        .collect();
    let text_channels: Vec<_> = channels.iter()
        .filter(|x| if let ChannelType::Text = x.kind {
                    true
                } else {
                    false
                })
        .collect();

    let text_id = text_channels.iter()
        .find(|ch| ch.name == "bot")
        .expect("No text channel available")
        .id;

    let voice_id = voice_channels.iter()
        .find(|channel| channel.name == "General")
        .expect("No voice channel available")
        .id;

    let (mut connection, _) = discord.connect()?;


    discord.send_message(text_id, "Mit `!play {link}` macht ihr was!", "", false)?;

    let play_regex = regex::Regex::new(r"^!play\s(.*)$")?;

    loop {
        use discord::model::Event;
        match connection.recv_event() {
            Ok(event) => {
                match event {
                    Event::MessageCreate(ref message) if message.content == "!kys" => break,
                    Event::MessageCreate(ref message) if message.content == "!quit" => {
                        let voice = connection.voice(Some(server_id));
                        voice.stop();
                        voice.disconnect();
                    }
                    Event::MessageCreate(ref message) => {
                        for cap in play_regex.captures_iter(&message.content) {
                            let voice = connection.voice(Some(server_id));
                            voice.connect(voice_id);
                            println!("{}", &cap[1]);
                            discord::voice::open_ytdl_stream(&cap[1])
                                .map(|audio| voice.play(audio))
                                .ok();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    connection.drop_voice(Some(server_id));

    connection.shutdown()?;



    Ok(())
}


quick_main!(run);
