# Readme for Text-to-Podcast Web Application

## Overview
This web application is designed to convert text input into a podcast-style audio format. Utilizing the capabilities of OpenAI's GPT models and text-to-speech technology, the application creates dynamic, two-speaker podcast dialogues from the provided text. This innovative tool is perfect for transforming written content into engaging, auditory experiences, making it suitable for educational, entertainment, or content creation purposes.

## Features
- **Text-to-Podcast Conversion**: Converts user-inputted text into a lively, two-speaker podcast.
- **Dynamic Speaker Personalities**: Crafted using GPT models to ensure unique speaking styles and tones.
- **Audio Output**: Generates an audio file that can be played directly on the website.
- **User-Friendly Interface**: Simple and intuitive web interface for easy interaction.

## How to Run the Project

### Prerequisites
Before running the project, ensure you have the following installed:

Rust Programming Language: Ensure Rust is installed on your system. You can download it from the official Rust website.

Cargo: Rust's package manager, Cargo, is usually installed with Rust. You can verify its installation by running cargo --version in your terminal.

OpenAI API Key: Since the application uses OpenAI's APIs, you'll need an API key from OpenAI. You can obtain it by creating an account on OpenAI's platform and following their API access guidelines.


### Setting Up the Environment
1. **Clone the Repository**: Clone the project repository to your local machine.
2. **Environment Variables**: Set up the `.env` file in the project root with the following key:
   ```
   OPENAI_API_KEY=your_openai_api_key
   ```
   Replace `your_openai_api_key` with your actual OpenAI API key.

### Running the Server
1. Navigate to the project directory in your terminal.
2. Run the command `cargo run` to start the server.
3. The server will start on `http://127.0.0.1:8080`. Open this address in your web browser.

### Using the Application
1. On the web interface, enter the text you wish to convert into the provided textarea.
2. Click the "Create Podcast" button.
3. The application will process your input and generate a podcast-style audio file, which will then be available for playback directly on the webpage.

### Stopping the Server
- To stop the server, simply press `Ctrl + C` in your terminal where the server is running.

## Contributions
Contributions to the project are welcome. Please follow the standard fork-and-pull request workflow. Don't forget to add tests for new features and ensure that your code complies with the existing style.

## License
This project is licensed under the [MIT License](LICENSE.md). Please see the `LICENSE.md` file in the project root for more information.

## Support
For support, feature requests, or bug reports, please open an issue in the project's GitHub repository.

---

This README provides a comprehensive guide to setting up and using the Text-to-Podcast web application. If there are any specific functionalities or additional details you'd like to include, feel free to let me know!