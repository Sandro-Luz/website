let lastScrollTop = 0;
const progress = document.getElementById('animation_progress_bar');
let width = 0;

function updateProgressBar() {
    const st = document.documentElement.scrollTop;
    const scrollHeight = document.documentElement.scrollHeight;
    const clientHeight = document.documentElement.clientHeight;
    
    if (st > lastScrollTop) {
        // Scrolling down, increase the progress bar
        if (width < 100) {
            width = (st / (scrollHeight - clientHeight)) * 100;
            progress.style.width = `${width}%`;
        }
    } else {
        // Scrolling up, decrease the progress bar
        if (width > 0) {
            width = (st / (scrollHeight - clientHeight)) * 100;
            progress.style.width = `${width}%`;
        }
    }

    lastScrollTop = st;
}

document.addEventListener('scroll', updateProgressBar);
