# bitcoin_explorer

## Environment Setup

To configure the application properly, you must create a `.env` file containing necessary environment variables. This file should be located in the `client` folder of your project.

### Creating the .env File

1. Navigate to the `client` folder in your project directory.
2. Create a new file named `.env` if it does not already exist.
3. Open the `.env` file with a text editor of your choice.
4. Add the following configurations to the file, substituting the placeholder values with your actual data:

   ```plaintext
#[blockchain api]
BLOCKCHAIN_API_KEY = "" 
CRYPTO_API_KEY = ""

#[database config]
DB_HOSTNAME = ""
DB_USERNAME = ""
DB_PASSWORD = ""
DB_DATABASE = ""
DB_BLOCK_TABLE = ""
DB_NEWS_TABLE = ""
