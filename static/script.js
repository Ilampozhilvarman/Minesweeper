document.querySelectorAll('input[type="range"]').forEach((slider) => {
    const output = document.getElementById(slider.id + '-value');
    const update = () => {
        output.textContent = slider.value;
    };
    slider.addEventListener('input', update);
    update();
});
