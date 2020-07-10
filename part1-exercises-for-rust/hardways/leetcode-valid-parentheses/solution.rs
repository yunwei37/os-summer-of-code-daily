impl Solution {
    pub fn is_valid(s :String) -> bool {
        let mut stack = Vec::new();
        let mut flag = true;
        for i in s.chars() {
            if i == ')' {
                match stack.pop() {
                    None => {
                        flag = false;
                    },
                    Some(i) => {
                        if i != 0 {
                            flag = false;
                        }
                    }
                }
            }else if i == '('{
                stack.push(0);
            }else if i=='}'{
                 match stack.pop() {
                    None => {
                        flag = false;
                    },
                    Some(i) => {
                        if i != 1 {
                            flag = false;
                        }
                    }
                }
            }else if i == '{'{
                stack.push(1);
            }else if i==']'{
                 match stack.pop() {
                    None => {
                        flag = false;
                    },
                    Some(i) => {
                        if i != 2 {
                            flag = false;
                        }
                    }
                }
            }else if i=='['{
                stack.push(2);
            }
        }
        stack.len() == 0 && flag
    }
}