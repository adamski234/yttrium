The current version of the library contains 27 keys, that implement most of the original's functionality.

| Key name | Description | Parameters |
| :- | :- | :- |
| attach | Adds a URL to a file to be attached to the message | Required, the file URL |
| ban | Bans the member from the server | Optional, reason, defaults to nothing<br>Optional, days to remove messages from, default 0<br>Optional, user id, defaults to the sender |
| channel | Returns information about a channel | Optional, indicates what property should be returned. Can be: `id`, `name`, `position`, `type` or `region` for voice channels. Defaults to `id`<br>Optional, channel ID to target |
| cond | A conditional that returns the second parameter if the first parameter is truthy, otherwis it returns the third parameter | Required, the condition to check<br>Required, the value to return if the first parameter is truthy<br>Optional, the value to return if the first parameter is not truthy |
| database_exists | Checks whether a database, or a database key, exists | Required, what to check for. Can be: db, key<br>Required, database name<br>Required if the first parameter is `key`, the key to check for in database |
| db_read | Reads a value from the database | Required, the database name<br>Required, the key name to retrieve |
| db_write_str | Writes text to database | Required, the database name<br>Required, the key name to insert to<br> Required, the text to insert |
| delete | Deletes messages | Optional, time after which to delete the messages, default 0<br>Optional, amount of messages to delete, default 1<br>Optional, user ID for filtering messages, default no filtering |
| everyone | Mentions everyone or here | Optional, set to `here` to mention `@here` |
| guild | Returns information about the guild | Required, property that should be returned. Can be: `id`, `owner`, `membercount`, `rolecount`, `channelcount`, `icon` |
| hasrole | Checks if a member has a role | Required, the user ID to check<br>Required, the role ID or name to check for |
| joined | Returns the timestamp of when the user joined | Optional, user ID to target |
| kick | Kicks a user from the server | Optional, the reason for the kick |
| math | Does mathematical computations | Required, the mathematical expression |
| mention | Retrieves a mention from the sent message | Required, the type of mention. Can be: `channels`, `users`, `roles`<br>Required, offset, starting with 0 |
| parameter | Returns the parameter passed with the message, or splits it on a string and returns that | Optional, the string to split on<br>Required, the index of the split string to return |
| pin | Pins the sent message | None |
| rand | Generates a random number | Optional, lowest value, default <br>Optional, highest value, default 10 |
| redirect | Chooses a channel to send the message to | Required, the channel ID to target |
| role | Adds a role to a member | Required, role ID or name to add<br>Optional, user ID to add the role to |
| selfdelete | Sets the time for removing the message sent by the bot | Required, the time after which the message should be deleted |
| selfreact | Adds a reaction to the message sent by the bot | Required, the reaction to add |
| setnickname | Sets a member's nickname | Required, nickname to change to<br>Optional, user ID to target |
| sleep | Pauses the script for some time | Required, the time to sleep for |
| take | Removes a role from a member | Required, the role ID or name to remove<br>Optional, user ID to remove the role from |
| text | Checks for string equality and similarity | Required, operation to perform. Can be: `equals`, `contains`, `starts_with`, `ends_with`, `regex`, `glob`, `extract`<br>Required, text to check<br>Required, text to check against. Regex if the first parameter is `regex`, glob if `glob` |
| trigger | Returns the trigger text | None |
| user | Retrieves information about the user | Optional, chooses information to return. Can be: `id`, `nickname`, `username`, `avatar`, `discriminator`<br>Optional, user ID to target