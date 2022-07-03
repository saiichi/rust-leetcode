// Mutex -> Semaphore

use tokio::sync::{Semaphore, SemaphorePermit};

pub struct Museum {
    remaining_tickets: Semaphore,
}

#[derive(Debug)]
pub struct Ticket<'a> {
    permit: SemaphorePermit<'a>,
}

impl Museum {
    pub fn new(tickets: usize) -> Self {
        Self {
            remaining_tickets: Semaphore::new(tickets),
        }
    }

    pub fn get_ticket(&self) -> Option<Ticket<'_>> {
       match self.remaining_tickets.try_acquire()  {
          Ok(permit) => Some(Ticket {
            permit,
          }),
          Err(_) => None,
       }
    }

    pub fn tickets(&self) -> usize {
        self.remaining_tickets.available_permits()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let museum = Museum::new(10);
        assert_eq!(museum.tickets(), 10);
        let ticket = museum.get_ticket();
        assert!(ticket.is_some());
        assert_eq!(museum.tickets(), 9);
        let _tickets: Vec<Ticket> = (0..9).map(|_| museum.get_ticket().unwrap()).collect();
        assert_eq!(museum.tickets(), 0);
        assert!(museum.get_ticket().is_none());
        drop(ticket);
        {
            let ticket = museum.get_ticket();
            println!("got ticket{:?}", ticket);
        }
    }
}