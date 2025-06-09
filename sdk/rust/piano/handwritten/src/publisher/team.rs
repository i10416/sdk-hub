mod schema;
pub use self::schema::*;

use crate::{PianoAPI, PianoResponse};

impl PianoAPI {
    /// List app team members
    ///
    /// Lists the team members added in the Team Manager of a given app.
    /// You can optionally filter team members by permissions.
    ///
    /// # Arguments
    ///
    /// * `params` - The list team request parameters
    ///
    /// # Returns
    ///
    /// Returns a list of team members.
    ///
    /// # Reference
    ///
    /// See the [Piano API documentation](https://docs.piano.io/api?endpoint=post~2F~2Fpublisher~2Fteam~2Flist) for more details.
    #[cfg_attr(feature = "tracing", tracing::instrument(skip(self)))]
    pub async fn list_team_members(
        &self,
        params: &ListTeamRequest,
    ) -> Result<TeamMemberListResult, crate::Error> {
        let result = self
            .client
            .get(format!("{}/publisher/team/list", self.endpoint))
            .query(&[("aid", &self.app_id)])
            .query(params)
            .send()
            .await?
            .json::<PianoResponse<TeamMemberListResult>>()
            .await?
            .value()?;
        Ok(result)
    }
}
