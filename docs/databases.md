# This is still WIP and currently only serves as a developer reference

# Database file format
Databases are stored in a JSON file, one per guild. In the top level there are keys that also serve as database names. Their value is an object with keys that serve as database keys. The values can be of any type you want, but if they aren't strings or arrays of strings, they will get stringified. Converting that data to the desired form is to be done by the key itself. Files should be named `guild_id.json` and reside in `databases` folder by default.

Example:
```json
{
	"database1": {
		"string_key": "string_value",
		"array_key": ["abc", "def"]
	}
}
```