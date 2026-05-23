-- Seed data for the Checkmate application.
--
-- Passwords are hashed with PBKDF2-HMAC-SHA512 (1 623 rounds, 32-byte key),
-- then base64-encoded, matching the scheme used by the registration handler.
--
-- Plaintext passwords (for reference / testing):
--   magnus_bishop    -> GrandmasterMagnus
--   tactical_tanya   -> TacticsTanya99
--   blundering_bob   -> BlunderBob1337
--   queen_gambette   -> QueenGambette!
--   rook_rupert      -> RookEndgame42
--   knight_nadia     -> NadiaJumps88
--   pawn_storm_pat   -> StormPat2024
--   fianchetto_fio   -> FioFianchetto!
--   en_passant_ed    -> EdEnPassant99
--   castling_cara    -> CaraCastles!
--   bishop_pair_bo   -> BoPairBishop
--   fork_felix       -> FelixForksU
--   zwischenzug_zoe  -> ZoeZwisch2025
--   skewer_simon     -> SimonSkewer!
--   pin_priya        -> PriyaPins42
--   zugzwang_zelda   -> ZeldaZugzwang!
--   overload_omar    -> OmarOverload77
--   tempo_tomas      -> TomaTempo1234
--   deflect_diana    -> DianaDeflects!
--   checkmate_chad   -> ChadCheckmate!!

INSERT OR IGNORE INTO users (username, password_pbkdf2_hash, password_salt, bio, rating) VALUES
  ('tactical_tanya',  'b9d25GXz+yElkeyVsf/Zca+d8aBa22C+7Dk0qJ8+cJw=', '1nDljgNR2K6OT26sNC/CMQ==', 'Queen sac enjoyer. En passant is a right.',     1750),
  ('blundering_bob',  'k0AthXpem53wPddHSB4jjmRX0vMK7QQq2ll938KjmbA=', 't7CHFus/wSiWuWIjF3SUKA==', 'It''s not a blunder, it''s a sacrifice.',        1200),
  ('queen_gambette',  'ZI6H0/IkXXktjhXfqgHggIjOcItNTjqAtrvGUGH3Kes=', 'dzPCjui6U721a4gkV31T7A==', 'd4 d5 c4. Simple.',                              1650),
  ('rook_rupert',     'kbxr98pqfL4f9tOcYP8yxvn/nz6NDS16W/gaNDhM4Pw=', 'wopwphx1EKHNiSFsoWz/yg==', 'Rooks belong on open files, like me.',          1580),
  ('knight_nadia',    'P6PI1EcGxJ9WFw5u/y89BWQNHihJH/Jv0kf8ZV9IG50=', '6kmHR36G28y5cEb8Lhg4Tg==', 'Knights outpost on d5 makes my heart race.',    1720),
  ('pawn_storm_pat',  'f9gHuMNtE3PSvtAdYW9Dc/Tc8SX6T0F31ktEzAGvFOo=', 'UdggxcPvgAU6iK45lt5Q6A==', 'My pawns march like an army.',                  1440),
  ('fianchetto_fio',  'NpZ82eTgDXLfMOjLEq6IuDOo9r0OX5lFABSJxU8PaKA=', 'AYZbNphlTr9SAKX6CTm5nQ==', 'Bishops on long diagonals. Always.',            1610),
  ('en_passant_ed',   '5J6qcg8vyVtdabYIZYz5vXcsh5vMKS3/59GRaYcMCwc=', 'eh17KCv4I0BB81SH2Gxmnw==', 'I will always capture en passant. Always.',     1300),
  ('castling_cara',   'NPKg15pPpV8SvsHimAhXUyRNHQbYTgG/qz1UniiGh1E=', 'zL/g5z1+cyCtCnVwAyQedQ==', 'Kingside or queenside? Yes.',                   1520),
  ('bishop_pair_bo',  'q5CCwp7Xgb+8i5bD6tkbQywlHETGvguBQHIpK52eYas=', 'IhCpJHmO+G1D8nzy0GEwMQ==', 'Two bishops are better than one partner.',      1680),
  ('fork_felix',      'JHehduiIWgfbZroyvRwjTbpnDZSJpCDZpO9RwTkgMHc=', '3LXY0u8bMh/OrTd/YmHlRw==', 'I fork kings for fun.',                         1800),
  ('zwischenzug_zoe', 'GZTOKXxZQAvbXkis4KzqZh3aqhSH+pAw0kdW1hfANqM=', '2F2O7H8m4jIZBy95VdD49g==', 'Intermediate moves saved my life.',             1760),
  ('skewer_simon',    'T4EoEihzL0NPpz0tOWNJ0UaBlM+ZW3BWk+WiOnNwenY=', 'bc0eVMIBx4foktj5T2GXbw==', 'Attack the stronger piece. Obviously.',         1540),
  ('pin_priya',       'AjnzpQPlmQHRkEjx5azhQmWcbgJw58Cs7g0tfPFv+M0=', 'HR+gHRn0UB0pXyMieM49fg==', 'Absolute pins are my love language.',           1690),
  ('zugzwang_zelda',  'CMNypgW1EhMxnm3v3oNq1oub+djTnkM+0XT3gHYcVEY=', 'FCnWoYVooHqHykOZ6qElBA==', 'I put you in zugzwang on the first date.',      1820),
  ('overload_omar',   'e9cFLxzrc5x+eHd/0l+uUuLMQ8J74PSxwaRHkm5OmUQ=', '6jMlbYdDsiN9vZFQ4JoEmQ==', 'Your pieces can''t defend everything.',          1630),
  ('tempo_tomas',     '3mHcj+0SzufXHqmwUhF91+I0DUtAW1x0EZVpKSfnAw4=', 'NUSHOzZPi5Brr2iH+oAaLw==', 'Every move must gain tempo.',                   1560),
  ('deflect_diana',   '2lbewWhXIJN08wVvtiqBQMfGrNzWBzrAJA88NlbpKdU=', '2I0WAapChlLi2gQ5JkwSvQ==', 'Deflection tactics are so romantic.',           1710),
  ('checkmate_chad',  'iD2JuZU3iC1l5wXa0aFTI9RUk9Jo+zvyQ1jXaAonIg8=', 'S9xBFZ26FLdrfzS10E95Uw==', 'My opening IS checkmate.',                      2100),
  ('rao',             'iD2JuZU3iC1l5wXa0aFTI9RUk9Jo+zvyQ1jXaAonIg8=', 'S9xBFZ26FLdrfzS10E95Uw==', 'Will you be a pawn to protect my intricate kingdom?', 9999),
  ('MrRook',          '9Me0oGCiNvZb9eO85D0JZivzgBk1reME7Dl0/txcklU=', 'FCdNj2Ds4xltNHXpKhB3yA==', 'Tired of boring opening lines? I''m a desperado that will do my best to bring us to the late game. Looking for my Caïssa!', 1200);

INSERT OR IGNORE INTO contact_info (username, phone, email, location) VALUES
  ('tactical_tanya',  '+1-555-0102', 'tanya@checkmate.gg',    'Moscow, Russia'),
  ('blundering_bob',  '+1-555-0103', 'bob@checkmate.gg',      'Chicago, IL'),
  ('queen_gambette',  '+1-555-0104', 'gambette@checkmate.gg', 'Paris, France'),
  ('rook_rupert',     '+1-555-0105', 'rupert@checkmate.gg',   'London, UK'),
  ('knight_nadia',    '+1-555-0106', 'nadia@checkmate.gg',    'Warsaw, Poland'),
  ('pawn_storm_pat',  '+1-555-0107', 'pat@checkmate.gg',      'Toronto, Canada'),
  ('fianchetto_fio',  '+1-555-0108', 'fio@checkmate.gg',      'Rome, Italy'),
  ('en_passant_ed',   '+1-555-0109', 'ed@checkmate.gg',       'Austin, TX'),
  ('castling_cara',   '+1-555-0110', 'cara@checkmate.gg',     'Berlin, Germany'),
  ('bishop_pair_bo',  '+1-555-0111', 'bo@checkmate.gg',       'Stockholm, Sweden'),
  ('fork_felix',      '+1-555-0112', 'felix@checkmate.gg',    'Madrid, Spain'),
  ('zwischenzug_zoe', '+1-555-0113', 'zoe@checkmate.gg',      'Vienna, Austria'),
  ('skewer_simon',    '+1-555-0114', 'simon@checkmate.gg',    'Amsterdam, Netherlands'),
  ('pin_priya',       '+1-555-0115', 'priya@checkmate.gg',    'Mumbai, India'),
  ('zugzwang_zelda',  '+1-555-0116', 'zelda@checkmate.gg',    'Prague, Czech Republic'),
  ('overload_omar',   '+1-555-0117', 'omar@checkmate.gg',     'Cairo, Egypt'),
  ('tempo_tomas',     '+1-555-0118', 'tomas@checkmate.gg',    'Lisbon, Portugal'),
  ('deflect_diana',   '+1-555-0119', 'diana@checkmate.gg',    'Buenos Aires, Argentina'),
  ('checkmate_chad',  '+1-555-0120', 'chad@checkmate.gg',     'New York, NY'),
  ('rao',             '+1-555-0000', 'rao@rao.rao',           'Rao''s Intricate Kingdom'),
  ('MrRook',          '+1-555-9733', 'leon.bonifasse@monsatan.ctf', 'FLAG-{7b8a881648447cd70180a775e531e218}');
