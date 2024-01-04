use std::collections::VecDeque;

#[derive(Debug)]
pub struct InfoMessage {
    pub counter: usize,
    pub title: String,
    pub message: String,
}

impl InfoMessage {
    pub fn new(title: String, message: String) -> InfoMessage {
        return InfoMessage {
            title,
            message,
            counter: 1,
        };
    }
    pub fn to_string(&self) -> String {
        if self.counter == 1 {
            return self.message.clone();
        }
        return format!("{} mal: {}", self.counter, self.message);
    }
}

#[derive(Debug)]
pub struct InfoQueue {
    pub queue: VecDeque<InfoMessage>,
    pub timer: usize,
}

impl InfoQueue {
    pub fn new() -> InfoQueue {
        return InfoQueue {
            queue: VecDeque::new(),
            timer: 30,
        };
    }

    pub fn queue(&mut self, title: String, message: String) {
        let head = self.head();
        if head.is_some() && head.unwrap().title == title && head.unwrap().message == message {
            self.queue.front_mut().unwrap().counter += 1;
        } else {
            let info = InfoMessage::new(title, message);
            self.queue.push_back(info);
            log::info!("{:?}", self.queue);
        }
    }

    pub fn dequeue(&mut self) {
        self.timer = 30;
        self.queue.pop_front();
    }

    pub fn head(&self) -> Option<&InfoMessage> {
        self.queue.front()
    }
}
