use log::debug;
use std::error::Error;

use crate::data_service::DataService;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, Me,
    },
    utils::command::BotCommands,
};

/// These commands are supported:
#[derive(BotCommands)]
#[command(rename_rule = "lowercase", description = "Доступные команды:")]
enum Command {
    /// Display this text
    #[command(description = r#"Помощь"#)]
    Help,
    /// Start
    #[command(description = r#"С чего бы начать"#)]
    Start,
    /// Schedule
    ///
    #[command(description = r#"Информация о группах"#)]
    Schedule,
}

// #[tokio::main]
pub(crate) async fn run_bot(serv: DataService) -> Result<(), Box<dyn Error>> {
    log::info!("Starting buttons bot...");
    let map = dptree::deps![serv.clone()];

    let bot = Bot::from_env();

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(map)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    Ok(())
}

fn make_keyboard(srv: DataService) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];
    let groups = srv.data();

    debug!("load groups   : {:?}", groups);
    for versions in groups.chunks(1) {
        let row = versions
            .iter()
            .map(|version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
    srv: DataService,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                bot.send_message(msg.chat.id, Command::descriptions().to_string())
                    .await?;
            }
            Ok(Command::Schedule) => {
                let keyboard = make_keyboard(srv);
                bot.send_message(msg.chat.id, "Выберете группу")
                    .reply_markup(keyboard)
                    .await?;
            }

            Ok(Command::Start) => {
                let keyboard = make_keyboard(srv);
                bot.send_message(msg.chat.id, "Выберете группу")
                    .reply_markup(keyboard)
                    .await?;
            }

            Err(_) => {
                bot.send_message(msg.chat.id, "Command not found!").await?;
            }
        }
    }

    Ok(())
}

async fn inline_query_handler(
    bot: Bot,
    q: InlineQuery,
    srv: DataService,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let choice_group = InlineQueryResultArticle::new(
        "0",
        "Выберете группу",
        InputMessageContent::Text(InputMessageContentText::new("Список групп:")),
    )
    .reply_markup(make_keyboard(srv));

    bot.answer_inline_query(q.id, vec![choice_group.into()])
        .await?;

    Ok(())
}

async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    srv: DataService,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(selected_course) = q.data {
        log::info!("You chose: {}", selected_course);
        let course = srv
            .get_sched(&selected_course)
            .iter()
            .take(5)
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        if let Some(Message { id, chat, .. }) = q.message {
            bot.edit_message_text(chat.id, id, course).await?;
        } else if let Some(id) = q.inline_message_id {
            bot.edit_message_text_inline(id, course).await?;
        }
    }

    Ok(())
}
