// global variable
pub const REACTIONS_WITH_ME: &str = r#"
WITH reactions_with_me AS (
    SELECT
        reactions.reaction_id,
        reactions.message_id,
        reactions.emoji,
        reactions.reaction_count,
        bool_or(reaction_users.user_id = $1) AS me
    FROM
        reactions
    LEFT JOIN reaction_users ON reactions.reaction_id = reaction_users.reaction_id
    GROUP BY
        reactions.reaction_id, reactions.message_id, reactions.emoji, reactions.reaction_count
)
"#;

pub const SELECT_MESSAGES: &str = r#"
SELECT
    qm.message_id,
    qm.user_id,
    qm.content,
    qm.channel_id,
    qm.reception_status,
    qm.edited,
    qm.reply_to,
    u.name AS author_name,
    u.username AS author_username,
    COALESCE(
        json_agg(
            json_build_object(
                'reaction_id', reactions_with_me.reaction_id,
                'count', reactions_with_me.reaction_count,
                'me', reactions_with_me.me,
                'emoji', reactions_with_me.emoji
            )
        ) FILTER (WHERE reactions_with_me.message_id IS NOT NULL AND reactions_with_me.reaction_count > 0),
        '[]'
    ) AS reactions
"#;

pub fn select_messages_from(
    from: &str
) -> String {
    let string = format!(r#"
{},
querying_messages AS (
{}
)
{}
FROM querying_messages qm
LEFT JOIN reactions_with_me ON reactions_with_me.message_id = qm.message_id
LEFT JOIN users u ON qm.user_id = u.user_id
GROUP BY
    qm.message_id, qm.user_id, qm.content, qm.channel_id, qm.reception_status, qm.edited, qm.reply_to, u.name, u.username
ORDER BY
    qm.message_id DESC
"#, REACTIONS_WITH_ME, from, SELECT_MESSAGES);
    return string;
}