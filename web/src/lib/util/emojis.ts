const EMOJI_REGEX = /(\u00a9|\u00ae|[\u2000-\u3300]|\ud83c[\ud000-\udfff]|\ud83d[\ud000-\udfff]|\ud83e[\ud000-\udfff])/gi;

export function isMessageMadeOfOnlyEmojis(message: string): boolean {
  return message.replace(EMOJI_REGEX, '').length === 0;
}

export function countEmojis(message: string): number {
  return message.match(EMOJI_REGEX)?.length ?? 0;
}