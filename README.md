# et-vis

Data gathering and visualisation assignment for CS3012.

The visualisation is in two stages - a Rust program that polls the GitHub V3 API and proceeds to fill the database, and a Ruby on Rails instance that creates a server which hosts a webpage with the visualisation

### Requirements:

* Rust/Cargo
* MySQL
* Ruby, Ruby on Rails

Details used by the Rust program for API collection, such as the API token and MySQL user & password,
are stored in a file `DETAILS` in the root directory in the project, With the layout of the file given by `DETAILS.example`.

For Ruby on Rails, the database details need to be inserted in `et-vis/config/database.yml`
