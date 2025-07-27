
document.addEventListener('DOMContentLoaded', () => {
    document.querySelector('#submit').addEventListener('click', async () => {
        const username = document.querySelector('#username').value;
        const password = document.querySelector('#password').value;
        const responseElement = document.querySelector('.response');
        const errorElement = document.querySelector('.error');

        // Clear feedback
        responseElement.setAttribute('hidden', '');
        errorElement.setAttribute('hidden', '');

        if (username == '' || password == '') {
            errorElement.textContent = 'no username or password provided';
            errorElement.removeAttribute('hidden');
            return;
        }

        const response = await fetch(`/api/whitelist/${password}/${username}`);
        if (response.status != 200) {
            errorElement.textContent = await response.text();
            errorElement.removeAttribute('hidden');
            return;
        }

        responseElement.textContent = "Success!";
        responseElement.removeAttribute('hidden');
    })
});
