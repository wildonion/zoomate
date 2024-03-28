


## hoopoe, platform for social events:

- redis pubsub actors over ws to maintain realtime communication with clients
- http based sse (broadcaster.rs)
- spacetimedb to store chats and notifs
- aws and gem auth check token api to retrieve user data
- sqlx to update pg data 
- realtime streaming (audio/video), chatroom, social, wishlist and onchain collection events with entrance fee
- cmq to start the chat randomly between two online peers
- ci actors, middlewares, http api component actors