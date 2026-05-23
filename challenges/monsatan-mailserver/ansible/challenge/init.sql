-- Create tables
CREATE TABLE IF NOT EXISTS config (
    name varchar(255),
    value varchar(255) NOT NULL,
    PRIMARY KEY (name)
);

CREATE TABLE IF NOT EXISTS users (
    username varchar(255),
    password_hash varchar(255) NOT NULL,
    PRIMARY KEY (username)
);

CREATE TABLE IF NOT EXISTS mails (
    id int AUTO_INCREMENT,
    from_id varchar(255) NOT NULL,
    to_id varchar(255) NOT NULL,
    subject varchar(255) NOT NULL,
    body MEDIUMTEXT NOT NULL,
    sent_at varchar(255) NOT NULL,
    is_read int NOT NULL DEFAULT 0,
    PRIMARY KEY (id),
    FOREIGN KEY (from_id) REFERENCES users(username),
    FOREIGN KEY (to_id) REFERENCES users(username)
);

-- Insert initial data
INSERT INTO users (username, password_hash) VALUES
('johann.elke', '$argon2id$v=19$m=19456,t=2,p=1$dgPaCqNnamJE/Nnwda5ETw$J3O0ZO/MmlVn7zCpKKWTgznIbdKfPoXOfqYvW/l6oEo'),
('bmcknight', '$argon2id$v=19$m=19456,t=2,p=1$3NzbhB77t/u3r87u96yheA$K2L9G7WMDDO4aswqEV/ORllEujthm4PLkMA89/QF8Kw'),
('ahamilton', '$argon2id$v=19$m=19456,t=2,p=1$RXGstSV7j01VyABAGoqefg$w+1e5cTEdSYA9Yovt2C/PPkywR1VNOC+KRGfiD3Y0YI'),
('mirwin', '$argon2id$v=19$m=19456,t=2,p=1$3HD9pcvC8NhMYqpYHEkaXg$t3dTPA6Z2+dnCrYarmbvkKFjux2e9MRrLabHsW8WaUc'),
('accounting', '$argon2id$v=19$m=19456,t=2,p=1$DrpCHizsnMlH+pu1kITYbg$W3x5Mnpes2GnN3Zfp2KsyIlHZZRbiMGrEmn9Hv34Vew');

INSERT INTO mails (from_id, to_id, subject, body, sent_at) VALUES
('johann.elke', 'bmcknight', 'Meeting', 'Hi Brian, let me know if you can make it.', '2026-01-01 10:00:00'),
('bmcknight', 'johann.elke', 'Re: Meeting', 'Sure, I can make it.', '2026-01-01 10:05:00'),
('ahamilton', 'johann.elke', 'Project Update', 'The project is on track.', '2026-01-02 09:00:00'),
('johann.elke', 'ahamilton', 'Re: Project Update', 'Thanks for the update.', '2026-01-02 09:05:00'),
('mirwin', 'johann.elke', 'Inquiry', 'I have a question about the project.', '2026-01-03 11:00:00'),
('johann.elke', 'mirwin', 'Re: Inquiry', 'Sure, I can help with that.', '2026-01-03 11:05:00'),
('johann.elke', 'mirwin', 'Re: Inquiry', 'Sure, I can help with that.', '2026-01-03 11:05:00'),
('accounting', 'johann.elke', 'Re: Inquiry [CONFIDENTIAL]', 'Hello sir,\n\nFollowing the request from legal, here''s the document that the financial auditors requested. There is no evidence that government grants were used in embezzling for the purchase of private jets. We should be cleared for the audit.\n\nPlease note that the document is offered as-is, including your address, personal phone number, SSN and your offshore bank accounts transactions, both to you and to a third party which could damage your reputation if revealed.\n\nRegards,\nAccounting\n\nAttachments:FLAG-{ada5d92b4247a947d7d0e6dd52a9bb55}', '2026-05-17 11:35:43');
