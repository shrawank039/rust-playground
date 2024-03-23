#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increment_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
    fn change_username(&mut self, new_username: &str) {
        self.username = String::from(new_username);
    }
}

fn change_username(user: &mut User, new_username: &str){
    user.username = String::from(new_username);
}



#[cfg(test)]
mod test{
    use super::*;
#[test]
fn test_struct(){
    let mut user1: User = User {
        username: String::from("Shrawan"),
        email: String::from("shrawan@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    change_username(&mut user1, "Shrawan2");
    dbg!(user1);

    let mut user2: User = User {
        username: String::from("user"),
        email: String::from("user2@gmail.com"),
        sign_in_count: 0,
        active: false,
    };

    user2.change_username("user2");
    user2.increment_sign_in_count();
    dbg!(user2);
}
}

