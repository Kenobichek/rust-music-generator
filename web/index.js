import init, { generate_samples_wasm } from "./pkg/rust_music_generator.js";

const NOTES = {
    "C4": 261.63, "C#4": 277.18, "D4": 293.66, "D#4": 311.13,
    "E4": 329.63, "F4": 349.23, "F#4": 369.99, "G4": 392.00,
    "G#4": 415.30, "A4": 440.00, "A#4": 466.16, "B4": 493.88,
    "C5": 523.25
};

const audioCtx = new (window.AudioContext || window.webkitAudioContext)();

function playNote(note, waveName, duration = 0.2) {
    const freq = NOTES[note];
    if (!freq) return;

    // Используем новую функцию из WASM
    const samples = generate_samples_wasm([freq], [duration], waveName);

    const buffer = audioCtx.createBuffer(1, samples.length, audioCtx.sampleRate);
    buffer.copyToChannel(new Float32Array(samples), 0);

    const source = audioCtx.createBufferSource();
    source.buffer = buffer;
    source.connect(audioCtx.destination);
    source.start();
}

function setupKeyboard() {
    const keyboard = document.getElementById("keyboard");
    const waveSelect = document.getElementById("wave-select");

    keyboard.addEventListener("click", (e) => {
        const key = e.target.closest(".key");
        if (!key) return;
        playNote(key.dataset.note, waveSelect.value);
    });
}

async function initSynth() {
    await init();
    setupKeyboard();
}

initSynth();
