use structs::common::{Taiga, APIError, ObjectType, DeleteProxy};

impl<'a> DeleteProxy<'a> {
    pub fn new(taiga_client: &'a Taiga, object_type: ObjectType, id: i64) -> DeleteProxy<'a> {
        DeleteProxy {
            taiga_client: taiga_client,
            object_type: object_type,
            object_id: id
        }
    }

    pub fn run(self: DeleteProxy<'a>) -> Result<i64, APIError> {
        // TODO
        return Ok(0);
    }
}
