use rand::prelude::*;
use iced::executor;
use iced::widget::{Button, Column, Text, TextInput, Container, scrollable};
use iced::{Element, Command, Application, Length, Theme};

pub struct MainWindow {
    hidden_value: i32,
    user_guess_text: String,
    guess_handoff: String,
    hidden_compare: i32,
    number_ofguesses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    BtnGuessNow,
    UserInputValueUpdate(String),
}

impl MainWindow {
    fn guess_output_calc(&self) -> String {
        let mut tempoutput = String::new();

        for (i, item) in self.number_ofguesses.iter().enumerate().skip(1) {

            let guessfmt = format!("Guess# {} Was: {}\n", i, item);
            tempoutput.push_str(&guessfmt);

        };

        tempoutput
    }
}

impl Application for MainWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        let y = rand::thread_rng().gen_range(1..100);

        let nguess: Vec<String> = vec!("".to_string());
        
        (
            MainWindow {
                hidden_value: y, 
                user_guess_text: String::from(""),
                guess_handoff: String::from("A guessing game example from Rust \"The Book\" ! Input a number between 1-100"),
                hidden_compare: 0,
                number_ofguesses: nguess,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Guess GUI - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::BtnGuessNow => {
                match self.user_guess_text.trim().parse::<i32>() {
                    Ok(value) => {
                        self.hidden_compare = value;
                        if self.hidden_value == self.hidden_compare {
                            self.number_ofguesses.push(self.hidden_compare.to_string() + ", A WINNER!");
                            self.guess_handoff = self.guess_output_calc();
                            self.user_guess_text = "".to_string();
                        } 
                        else if self.hidden_value > self.hidden_compare {
                            self.number_ofguesses.push(self.hidden_compare.to_string() + ", Too Low!");
                            self.guess_handoff = self.guess_output_calc();
                            self.user_guess_text = "".to_string();
                        } 
                        else if self.hidden_value < self.hidden_compare {
                            self.number_ofguesses.push(self.hidden_compare.to_string()+ ", Too Big!");
                            self.guess_handoff = self.guess_output_calc();
                            self.user_guess_text = "".to_string();
                        }
                    }
                    Err(_) => {
                        self.user_guess_text = "Please input a number 1-100".to_string();
                        self.number_ofguesses.push("Error: Input number 1-100".to_string());
                        self.guess_handoff = self.guess_output_calc();
                    }
                }

                Command::none()
            }

            Message::UserInputValueUpdate(x) => {
                self.user_guess_text = x;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let main_text_input = TextInput::new("Input Value:1-100", &self.user_guess_text, Message::UserInputValueUpdate)
            .size(25)
            .width(iced::Length::Units(175))
            .on_submit(Message::BtnGuessNow);

        let guess_button = Button::new("Guess now!").on_press(Message::BtnGuessNow).height(iced::Length::Units(28)).width(Length::Units(98));
    
        let user_output = Text::new(&self.guess_handoff).width(Length::Units(400));
        let user_output_scroll = scrollable(user_output).height(Length::Fill);
        
        let main_column = Column::new()
            .push(main_text_input)
            .push(guess_button)
            .push(user_output_scroll)
            .spacing(15)
            .padding(20)
            .max_width(400);

        let guess_layout = Container::new(main_column).width(iced::Length::Fill).height(iced::Length::Fill);

        guess_layout.into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}