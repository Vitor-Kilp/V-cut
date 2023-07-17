export default function parseTime(timeString: string) {
    const [hours, minutes, secondsWithMillis] = timeString.split(":");
    const [seconds, _] = secondsWithMillis.split(".");
    const totalSeconds =
        parseInt(hours) * 3600 +
        parseInt(minutes) * 60 +
        parseInt(seconds);
    return Math.floor(totalSeconds * 10) / 10;
}