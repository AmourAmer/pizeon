create table if not exists notices (
  blocked boolean not null,
	id text primary key,
	timestamp integer not null,
	body json not null,
	versions text not null,
	deleted_at integer,
  expires_at integer,
  last_changed integer

	-- unique(timestamp, cwd, command)
);

create index if not exists idx_notice_timestamp on notices(timestamp);
create index if not exists idx_notice_blocked on notices(blocked);

create index if not exists idx_history_blocked_timestamp on notices(
	blocked,
	timestamp
);
