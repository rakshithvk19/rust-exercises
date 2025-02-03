// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug)]
enum StatusErr {
    EmptyStatus,
    UnauthorizedStatus,
}

impl TryFrom<String> for Status {
    type Error = StatusErr;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("ToDo") {
            Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("InProgress") {
            Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("Done") {
            Ok(Status::Done)
        } else if value.is_empty() {
            Err(StatusErr::EmptyStatus)
        } else {
            Err(StatusErr::UnauthorizedStatus)
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = StatusErr;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("ToDo") {
            Ok(Status::ToDo)
        } else if value.eq_ignore_ascii_case("InProgress") {
            Ok(Status::InProgress)
        } else if value.eq_ignore_ascii_case("Done") {
            Ok(Status::Done)
        } else if value.is_empty() {
            Err(StatusErr::EmptyStatus)
        } else {
            Err(StatusErr::UnauthorizedStatus)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
