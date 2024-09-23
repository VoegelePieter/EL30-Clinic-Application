/**
 * Displays a simple browser alert message.
 * @param {string} message - The message to display in the alert.
 */
export const showAlert = (message) => {
    alert(message);
};

/**
 * Formats an ISO datetime string into HH:MM format.
 * @param {string} datetime - The ISO datetime string.
 * @returns {string} - The formatted time string.
 */
export const formatTime = (datetime) => {
    const dateObj = new Date(datetime);
    const hours = dateObj.getHours().toString().padStart(2, '0');
    const minutes = dateObj.getMinutes().toString().padStart(2, '0');
    return `${hours}:${minutes}`;
};

/**
 * Calculates the duration of an appointment in minutes.
 * @param {string} start - Start time in ISO format.
 * @param {string} end - End time in ISO format.
 * @returns {number} - Duration in minutes.
 */
export const calculateLength = (start, end) => {
    const startTime = new Date(start);
    const endTime = new Date(end);
    const diffInMs = endTime - startTime;
    return Math.round(diffInMs / 60000);
};

/**
 * Capitalizes the first letter of a string.
 * @param {string} string - The string to capitalize.
 * @returns {string} - The capitalized string.
 */
export const capitalizeFirstLetter = (string) => {
    if (!string) return '';
    return string.charAt(0).toUpperCase() + string.slice(1);
};

/**
 * Validates a form based on HTML5 validation rules.
 * Adds the 'was-validated' class if validation fails.
 * @param {HTMLFormElement} form - The form to validate.
 * @returns {boolean} - Returns true if the form is valid, otherwise false.
 */
export const handleFormValidation = (form) => {
    if (!form.checkValidity()) {
        form.classList.add('was-validated');
        return false;
    }
    form.classList.remove('was-validated');
    return true;
};

/**
 * Populates a dropdown menu with options.
 * @param {HTMLSelectElement} selectElement - The dropdown element to be populated.
 * @param {Array} items - The array of items to be inserted.
 * @param {string} defaultOptionText - The text for the default option.
 * @param {boolean} addPrefix - Whether to add a prefix to the values (e.g., "patient:").
 */
export const populateDropdown = (selectElement, items, defaultOptionText, addPrefix) => {
    selectElement.innerHTML = `<option value="" disabled selected>${defaultOptionText}</option>`;
    items.forEach(item => {
        const option = document.createElement('option');
        option.value = addPrefix ? `patient:${item.id.id.String}` : item.id.id.String;
        option.textContent = item.name;
        selectElement.appendChild(option);
    });
};

/**
 * Populates the room dropdown based on the number of rooms available.
 * @param {HTMLSelectElement} selectElement - The dropdown element for rooms.
 * @param {number} roomAmount - The number of available rooms.
 */
export const populateRoomDropdown = (selectElement, roomAmount) => {
    selectElement.innerHTML = '<option value="" disabled selected>Select a Room</option>';
    for (let i = 0; i <= roomAmount; i++) { // Start bei 0, um alle Räume abzudecken
        const option = document.createElement('option');
        option.value = i;
        option.textContent = `Room ${i}`;
        selectElement.appendChild(option);
    }
    console.log('Room Dropdown Populated');
};

/**
 * Populates the start time dropdown with predefined, hardcoded time options.
 * @param {HTMLSelectElement} selectElement - The dropdown element for start time.
 */
export const populateStartTimeDropdown = (selectElement) => {
    const timeSlots = [
        { start: '08:00', end: '13:00' },
        { start: '14:00', end: '17:00' }
    ];

    selectElement.innerHTML = '<option value="" disabled selected>Select Start Time</option>';

    timeSlots.forEach(interval => {
        let [startHour, startMinute] = interval.start.split(':').map(Number);
        let [endHour, endMinute] = interval.end.split(':').map(Number);
        let startTime = new Date(1970, 0, 1, startHour, startMinute);
        const endTime = new Date(1970, 0, 1, endHour, endMinute);

        while (startTime < endTime) {
            const hours = startTime.getHours().toString().padStart(2, '0');
            const minutes = startTime.getMinutes().toString().padStart(2, '0');
            const timeString = `${hours}:${minutes}`;

            const option = document.createElement('option');
            option.value = `${timeString}:00`;
            option.textContent = timeString;
            selectElement.appendChild(option);

            startTime.setMinutes(startTime.getMinutes() + 30);
        }
    });

    console.log('Start Time Dropdown Populated');
};
