export const extract_session_id = (input_text) => {
	const sessionIdRegex = /session_id:\s+"(.*?)"/;
	const sessionIdMatch = input_text.match(sessionIdRegex);
	const sessionId = sessionIdMatch ? sessionIdMatch[1] : null;
	return sessionId;
};

export const extract_auth_code = (input_text) => {
	const codeRegex = /code:\s+"(.*?)"/;
	const codeMatch = input_text.match(codeRegex);

	const code = codeMatch ? codeMatch[1] : null;
	return code;
};
