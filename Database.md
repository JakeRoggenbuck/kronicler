## Database

Kronicler includes a custom columnar database to store and run calculations on logs.

### Data Format

Each column in the database stores a specific data field in a separate file.

| Column Name | Rust Data Type | Stored-as Data Type | Description                                      |
|-------------|----------------|---------------------|--------------------------------------------------|
| `name`      | `String`       | `[u8; 64]`          | The function name being logged                   |
| `start`     | `Epoch (u128)` | `u128`              | The epoch time that the function starts running  |
| `end`       | `Epoch (u128)` | `u128`              | The epoch time that the function ends running    |
| `delta`     | `Epoch (u128)` | `u128`              | The epoch duration that the function was running |
