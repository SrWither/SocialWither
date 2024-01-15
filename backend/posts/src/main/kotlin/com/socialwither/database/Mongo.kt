package com.socialwither.databases.Mongo

import kotlinx.serialization.*
import kotlinx.coroutines.withContext
import kotlinx.coroutines.Dispatchers
import org.litote.kmongo.reactivestreams.*
import org.litote.kmongo.coroutine.*
import com.socialwither.models.Post.*
import com.mongodb.client.result.DeleteResult
import org.litote.kmongo.Id
import org.litote.kmongo.id.StringId
import org.litote.kmongo.id.toId
import org.bson.types.ObjectId

/**
 * Service class for interacting with the "posts" collection in the "SocialWither" MongoDB database.
 */
class PostService {
    // MongoDB connection configuration
    private val connectionString = "mongodb://root:root@localhost:27017/"
    private val client = KMongo.createClient(connectionString).coroutine
    private val database = client.getDatabase("SocialWither")

    // Collection reference for the "posts" collection
    private val postCollection: CoroutineCollection<Post> = database.getCollection<Post>("posts")

    /**
     * Creates a new post in the "posts" collection.
     *
     * @param post The Post object to be inserted.
     * @return PostDto representation of the created post.
     */
    suspend fun create(post: Post): PostDto? {
        return withContext(Dispatchers.IO) {
            postCollection.insertOne(post)
            return@withContext post.toDto()
        }
    }

    /**
     * Retrieves a post by its ID from the "posts" collection.
     *
     * @param id The ID of the post to retrieve.
     * @return PostDto representation of the retrieved post.
     */
    suspend fun get(id: String): PostDto? {
        return withContext(Dispatchers.IO) {
            val bsonId: Id<Post> = ObjectId(id).toId()
            val post: Post? = postCollection.findOneById(bsonId)
            return@withContext post?.toDto()
        }
    }

    /**
     * Retrieves all posts from the "posts" collection.
     *
     * @return List of PostDto representations of all posts.
     */
    suspend fun getAll(): List<PostDto> {
        return withContext(Dispatchers.IO) {
            val posts: List<Post> = postCollection.find().toList()
            return@withContext posts.mapNotNull { it.toDto() }
        }
    }

    /**
     * Updates a post in the "posts" collection.
     *
     * @param id The ID of the post to update.
     * @param post The updated Post object.
     * @return PostDto representation of the updated post.
     */
    suspend fun update(id: String, post: Post): PostDto? {
        return withContext(Dispatchers.IO) {
            val bsonId: Id<Post> = ObjectId(id).toId()
            val updateResult = postCollection.updateOneById(bsonId, post.markAsEdited())
            if (!updateResult.wasAcknowledged()) {
                return@withContext null
            }
            val updatedPost = postCollection.findOneById(bsonId)
            return@withContext updatedPost?.toDto()
        }
    }

    /**
     * Deletes a post from the "posts" collection.
     *
     * @param id The ID of the post to delete.
     * @return True if the deletion was successful, false otherwise.
     */
    suspend fun delete(id: String): Boolean {
        return withContext(Dispatchers.IO) {
            val bsonId: Id<Post> = ObjectId(id).toId()
            val postToDelete: Post? = postCollection.findOneById(bsonId)
    
            if (postToDelete != null) {
                val result = postCollection.deleteOneById(bsonId)
                return@withContext result.wasAcknowledged()
            } else {
                return@withContext false
            }
        }
    }    
}
