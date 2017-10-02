# sonar

_We're not just ships passing in the night; we're submarines lost in the depths, wholly invisible to each other until we actively ping._

`sonar` is a basic Twitter clone, intended to run on a free-tier AWS stack as a demonstration project for my portfolio. Still, maybe it could take off and turn me into an internet billionaire, right?

## Intended features

These features need to be implemented in order for me to consider this a complete demo project.

- [ ] user signup / authentication
- [ ] user profiles (handle, real name, brief bio)
- [ ] user can create a `ping`: short message up to 140 chars
- [ ] user view showing most recent pings
- [ ] follow another user
- [ ] timeline view showing your pings and those of those people you follow
- [ ] timeline will only ever be linear
- [ ] http addresses auto-expand into links
- [ ] individual ping permalink view
- [ ] individual ping replies view
- [ ] user tags link to user view
- [ ] mentions view showing people writing about you
- [ ] block another user (they cannot see you; you cannot see them)
- [ ] user notifications on tagging
- [ ] hashtags / hashtag search view

## Horizon features

These features would be great, but probably won't happen unless this starts to get a real userbase.

- [ ] users can 'like' pings
- [ ] liked pings view
- [ ] users can 'echo' (retweet) pings. probably just links to it; we don't want the one-button retweet culture from twitter.
- [ ] password reset via email feature
- [ ] email notifications on mentions
- [ ] general search
- [ ] report a ping/user (don't want to take twitter's cavalier attitude against the trolls)
- [ ] inline photos / video
- [ ] log in with twitter to import your contacts
- [ ] twitter bot using sentiment analysis and search to find tweets criticizing twitter, ideally for non-linear-timeline or terrible troll issues, and suggesting sonar as a replacement.

## Architecture

This project is the backend to the sonar website, implementing all features via a REST API. It's built in [rust](https://www.rust-lang.org/en-US/) using [rocket](https://rocket.rs/). I've done a lot of REST API development using Python/Django/DRF, and a lot of rust development, but this is my first time attempting to write an API in rust. The performance stats look good, and I really like Rust, so we'll see how this goes.

I'll build a frontend later, probably as a separate project.

The general idea is to build a simple, single-server implementation as proof of concept. Most likely, this will only ever run on AWS free tier machines, so we want to keep things simple. This will expect to talk to a single database without any sort of caching until demand makes something else necessary. The rocket framework promises that it will one day [handle async I/O for us transparently](https://github.com/SergioBenitez/Rocket/issues/17), and there's no point worrying about too much caching / sharding issues until the userbase takes off.
