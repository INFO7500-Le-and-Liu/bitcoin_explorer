# bitcoin_explorer

## Environment Setup

To configure the application properly, you must create a `.env` file containing necessary environment variables. This file should be located in the `client` folder of your project.

### Creating the .env File

1. Navigate to the `client` folder in your project directory.
2. Create a new file named `.env` if it does not already exist.
3. Open the `.env` file with a text editor of your choice.
4. Add the following configurations to the file, substituting the placeholder values with your actual data:

   ```plaintext
   # Blockchain API Configuration
   API_KEY = "56ddd388-5229-4dc4-b307-54d4d0f579c6"

   # Database Configuration
   DB_HOSTNAME = "localhost"
   DB_USERNAME = "root"
   DB_PASSWORD = "12345678"
   DB_DATABASE = "demo"
