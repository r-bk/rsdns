macro_rules! question_check_no_questions {
    ($self:ident) => {
        if $self.section_tracker.questions_left() == 0 {
            $self.done = true;
            return Err(Error::ReaderDone);
        }
    };
}

macro_rules! question_check_single_question {
    ($self:ident) => {
        if $self.section_tracker.questions_left() != 1 {
            $self.done = true;
            return Err(Error::BadQuestionsCount(
                $self.section_tracker.questions_left(),
            ));
        }
    };
}

macro_rules! question {
    ($self:ident, $check:ident) => {{
        if $self.done {
            return Err(Error::ReaderDone);
        }

        $check!($self);

        let res = $self.cursor.read();
        if res.is_ok() {
            $self.section_tracker.question_read($self.cursor.pos());
        } else {
            $self.done = true;
        }

        res
    }};
}
