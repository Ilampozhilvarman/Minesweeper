const params = new URLSearchParams(window.location.search);
const width = Number(params.get('width'));
const height = Number(params.get('height'));
const minePercent = Number(params.get('mines'));

const boardEl = document.getElementById('board');
const statusEl = document.getElementById('status');
const minesLeftEl = document.getElementById('mines-left');
const flagToggle = document.getElementById('flag-toggle');

const NUMBER_COLORS = [
    '', '#4f8cff', '#4cc38a', '#ff6b6b', '#a78bfa',
    '#f59e0b', '#22d3ee', '#e8e8ee', '#9ca3af',
];

let cells = [];
let revealed = [];
let flagged = [];
let elements = [];
let mineTotal = 0;
let safeLeft = 0;
let flagCount = 0;
let over = false;
let flagMode = false;

function neighbors(index) {
    const row = Math.floor(index / width);
    const col = index % width;
    const result = [];
    for (let dr = -1; dr <= 1; dr++) {
        for (let dc = -1; dc <= 1; dc++) {
            if (dr === 0 && dc === 0) continue;
            const r = row + dr;
            const c = col + dc;
            if (r >= 0 && r < height && c >= 0 && c < width) {
                result.push(r * width + c);
            }
        }
    }
    return result;
}

function draw(i) {
    const el = elements[i];
    el.className = 'cell';
    el.textContent = '';
    el.style.color = '';
    if (revealed[i]) {
        el.classList.add('revealed');
        if (cells[i] === -1) {
            el.classList.add('mine');
            el.textContent = '💣';
        } else if (cells[i] > 0) {
            el.textContent = cells[i];
            el.style.color = NUMBER_COLORS[cells[i]];
        }
    } else if (flagged[i]) {
        el.classList.add('flagged');
        el.textContent = '🚩';
    }
}

function updateMinesLeft() {
    minesLeftEl.textContent = mineTotal - flagCount;
}

function lose(clicked) {
    over = true;
    for (let i = 0; i < cells.length; i++) {
        if (cells[i] === -1) {
            revealed[i] = true;
            draw(i);
        }
    }
    elements[clicked].classList.add('exploded');
    statusEl.textContent = 'Boom! You hit a mine.';
}

function win() {
    over = true;
    for (let i = 0; i < cells.length; i++) {
        if (cells[i] === -1 && !flagged[i]) {
            flagged[i] = true;
            flagCount++;
            draw(i);
        }
    }
    updateMinesLeft();
    statusEl.textContent = 'You win!';
}

function reveal(start) {
    if (over || revealed[start] || flagged[start]) return;
    if (cells[start] === -1) {
        lose(start);
        return;
    }
    const stack = [start];
    while (stack.length > 0) {
        const i = stack.pop();
        if (revealed[i] || flagged[i]) continue;
        revealed[i] = true;
        safeLeft--;
        draw(i);
        if (cells[i] === 0) {
            for (const n of neighbors(i)) {
                if (!revealed[n] && cells[n] !== -1) stack.push(n);
            }
        }
    }
    if (safeLeft === 0) win();
}

function toggleFlag(i) {
    if (over || revealed[i]) return;
    flagged[i] = !flagged[i];
    flagCount += flagged[i] ? 1 : -1;
    updateMinesLeft();
    draw(i);
}

function buildGrid() {
    const size = Math.max(
        16,
        Math.min(32, Math.floor((window.innerWidth - 48) / width))
    );
    boardEl.style.setProperty('--cell', size + 'px');
    boardEl.style.gridTemplateColumns = `repeat(${width}, var(--cell))`;
    boardEl.textContent = '';
    elements = [];
    for (let i = 0; i < cells.length; i++) {
        const el = document.createElement('div');
        el.className = 'cell';
        el.dataset.index = i;
        boardEl.appendChild(el);
        elements.push(el);
    }
}

boardEl.addEventListener('click', (event) => {
    const el = event.target.closest('.cell');
    if (!el) return;
    const i = Number(el.dataset.index);
    if (flagMode) {
        toggleFlag(i);
    } else {
        reveal(i);
    }
});

boardEl.addEventListener('contextmenu', (event) => {
    event.preventDefault();
    const el = event.target.closest('.cell');
    if (!el) return;
    toggleFlag(Number(el.dataset.index));
});

flagToggle.addEventListener('click', () => {
    flagMode = !flagMode;
    flagToggle.textContent = 'Flag mode: ' + (flagMode ? 'on' : 'off');
    flagToggle.classList.toggle('active', flagMode);
});

async function start() {
    statusEl.textContent = 'Loading...';
    try {
        const response = await fetch(
            `/board?width=${width}&height=${height}&mines=${minePercent}`
        );
        if (!response.ok) throw new Error('bad response');
        const data = await response.json();
        cells = data.cells;
    } catch (error) {
        statusEl.textContent = 'Could not load the board.';
        return;
    }
    revealed = new Array(cells.length).fill(false);
    flagged = new Array(cells.length).fill(false);
    mineTotal = cells.filter((c) => c === -1).length;
    safeLeft = cells.length - mineTotal;
    flagCount = 0;
    over = false;
    updateMinesLeft();
    buildGrid();
    statusEl.textContent = '';
}

start();
