package com.socialwither.models.Post

import kotlinx.serialization.*
import org.bson.codecs.pojo.annotations.BsonId
import org.litote.kmongo.*

/**
 * Data class representing a post entity.
 *
 * @property id The ID of the post.
 * @property user_id The user ID associated with the post.
 * @property title The title of the post.
 * @property content The content of the post.
 * @property edited A flag indicating whether the post has been edited.
 */
data class Post(
    @BsonId
    val id: Id<Post>? = null,
    val user_id: String,
    val title: String,
    val content: String,
    val edited: Boolean = false
) {
    /**
     * Marks the post as edited by creating a new instance with the 'edited' flag set to true.
     *
     * @return A new Post instance with 'edited' set to true.
     */
    fun markAsEdited(): Post {
        return copy(edited = true)
    }
}

/**
 * Data class representing a DTO (Data Transfer Object) for a post.
 *
 * @property id The ID of the post.
 * @property user_id The user ID associated with the post.
 * @property title The title of the post.
 * @property content The content of the post.
 * @property edited A flag indicating whether the post has been edited.
 */
@Serializable
data class PostDto(
    val id: String? = null,
    val user_id: String,
    val title: String,
    val content: String,
    val edited: Boolean = false
) {
    /**
     * Marks the DTO as edited by creating a new instance with the 'edited' flag set to true.
     *
     * @return A new PostDto instance with 'edited' set to true.
     */
    fun markAsEdited(): PostDto {
        return copy(edited = true)
    }

    fun setUserId(user_id: String): PostDto {
        return copy(user_id = user_id)
    }
}

/**
 * Extension function to convert a Post object to a PostDto.
 *
 * @return PostDto representation of the Post.
 */
fun Post.toDto(): PostDto =
    PostDto(
        id = this.id.toString(),
        user_id = this.user_id,
        title = this.title,
        content = this.content,
        edited = this.edited
    )

/**
 * Extension function to convert a PostDto object to a Post.
 *
 * @return Post representation of the PostDto.
 */
fun PostDto.toPost(): Post =
    Post(
        user_id = this.user_id,
        title = this.title,
        content = this.content,
        edited = this.edited
    )
