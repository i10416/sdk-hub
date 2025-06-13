final case class ListContractRequest(
  licensee_id: String,
  q: Option[String] = None,
  limit: Option[Int] = None,
  offset: Option[Int] = None,
)
object ListContractRequest:
  implicit val de: Decoder[ListContractRequest] = Decoder.forProduct4[ListContractRequest,String,Option[String],Option[Int],Option[Int]]("licensee_id","q","limit","offset")(ListContractRequest.apply)
  implicit val en: Encoder[ListContractRequest] = Encoder.forProduct4[ListContractRequest,String,Option[String],Option[Int],Option[Int]]("licensee_id","q","limit","offset")(p => (p.licensee_id,p.q,p.limit,p.offset))

final case class CreateContractRequest(
  licensee_id: String,
  contract_type: Option[PianoContractType] = None,
  contract_name: String,
  seats_number: Int,
  is_hard_seats_limit_type: Boolean,
  rid: String,
  landing_page_url: Option[String],
)
object CreateContractRequest:
  implicit val de: Decoder[CreateContractRequest] = Decoder.forProduct7[CreateContractRequest,String,Option[PianoContractType],String,Int,Boolean,String,Option[String]]("licensee_id","contract_type","contract_name","seats_number","is_hard_seats_limit_type","rid","landing_page_url")(CreateContractRequest.apply)
  implicit val en: Encoder[CreateContractRequest] = Encoder.forProduct7[CreateContractRequest,String,Option[PianoContractType],String,Int,Boolean,String,Option[String]]("licensee_id","contract_type","contract_name","seats_number","is_hard_seats_limit_type","rid","landing_page_url")(p => (p.licensee_id,p.contract_type,p.contract_name,p.seats_number,p.is_hard_seats_limit_type,p.rid,p.landing_page_url))

final case class UpdateContractRequest(
  licensee_id: String,
  contract_id: String,
  contract_description: Option[String] = None,
  contract_type: PianoContractType,
  contract_name: String,
  seats_number: Int,
  is_hard_seats_limit_type: Boolean,
  rid: String,
  landing_page_url: Option[String],
  schedule_id: Option[String] = None,
)
object UpdateContractRequest:
  implicit val de: Decoder[UpdateContractRequest] = Decoder.forProduct10[UpdateContractRequest,String,String,Option[String],PianoContractType,String,Int,Boolean,String,Option[String],Option[String]]("licensee_id","contract_id","contract_description","contract_type","contract_name","seats_number","is_hard_seats_limit_type","rid","landing_page_url","schedule_id")(UpdateContractRequest.apply)
  implicit val en: Encoder[UpdateContractRequest] = Encoder.forProduct10[UpdateContractRequest,String,String,Option[String],PianoContractType,String,Int,Boolean,String,Option[String],Option[String]]("licensee_id","contract_id","contract_description","contract_type","contract_name","seats_number","is_hard_seats_limit_type","rid","landing_page_url","schedule_id")(p => (p.licensee_id,p.contract_id,p.contract_description,p.contract_type,p.contract_name,p.seats_number,p.is_hard_seats_limit_type,p.rid,p.landing_page_url,p.schedule_id))

final case class ArchiveContractRequest(
  contract_id: String,
)
object ArchiveContractRequest:
  implicit val de: Decoder[ArchiveContractRequest] = Decoder.forProduct1[ArchiveContractRequest,String]("contract_id")(ArchiveContractRequest.apply)
  implicit val en: Encoder[ArchiveContractRequest] = Encoder.forProduct1[ArchiveContractRequest,String]("contract_id")(p => (p.contract_id))

final case class DeactivateContractRequest(
  contract_id: String,
)
object DeactivateContractRequest:
  implicit val de: Decoder[DeactivateContractRequest] = Decoder.forProduct1[DeactivateContractRequest,String]("contract_id")(DeactivateContractRequest.apply)
  implicit val en: Encoder[DeactivateContractRequest] = Encoder.forProduct1[DeactivateContractRequest,String]("contract_id")(p => (p.contract_id))

final case class ListContractResult(
  contracts: List[Contract],
)
object ListContractResult:
  implicit val de: Decoder[ListContractResult] = Decoder.forProduct1[ListContractResult,List[Contract]]("contracts")(ListContractResult.apply)
  implicit val en: Encoder[ListContractResult] = Encoder.forProduct1[ListContractResult,List[Contract]]("contracts")(p => (p.contracts))

sealed trait PianoContractType:
  def value: String

object PianoContractType:
  implicit val en: Encoder[PianoContractType] = Encoder[String].contramap[PianoContractType](_.value)
  case object SPECIFIC_EMAIL_ADDRESSES_CONTRACT extends PianoContractType:
    def value: String = "SPECIFIC_EMAIL_ADDRESSES_CONTRACT"

  case object EMAIL_DOMAIN_CONTRACT extends PianoContractType:
    def value: String = "EMAIL_DOMAIN_CONTRACT"

  case object IP_RANGE_CONTRACT extends PianoContractType:
    def value: String = "IP_RANGE_CONTRACT"


final case class ContractResult(
  contract: Contract,
)
object ContractResult:
  implicit val de: Decoder[ContractResult] = Decoder.forProduct1[ContractResult,Contract]("contract")(ContractResult.apply)
  implicit val en: Encoder[ContractResult] = Encoder.forProduct1[ContractResult,Contract]("contract")(p => (p.contract))

final case class Contract(
  contract_id: String,
  aid: String,
  contract_type: PianoContractType,
  name: String,
  description: Option[String] = None,
  create_date: Long,
  landing_page_url: Option[String] = None,
  licensee_id: String,
  seats_number: Int,
  is_hard_seats_limit_type: Boolean,
  rid: String,
  schedule_id: Option[String] = None,
  contract_is_active: Boolean,
  contract_periods: List[ContractPeriod] = Nil,
  contract_conversions_count: Int,
)
object Contract:
  implicit val de: Decoder[Contract] = Decoder.forProduct15[Contract,String,String,PianoContractType,String,Option[String],Long,Option[String],String,Int,Boolean,String,Option[String],Boolean,List[ContractPeriod],Int]("contract_id","aid","contract_type","name","description","create_date","landing_page_url","licensee_id","seats_number","is_hard_seats_limit_type","rid","schedule_id","contract_is_active","contract_periods","contract_conversions_count")(Contract.apply)
  implicit val en: Encoder[Contract] = Encoder.forProduct15[Contract,String,String,PianoContractType,String,Option[String],Long,Option[String],String,Int,Boolean,String,Option[String],Boolean,List[ContractPeriod],Int]("contract_id","aid","contract_type","name","description","create_date","landing_page_url","licensee_id","seats_number","is_hard_seats_limit_type","rid","schedule_id","contract_is_active","contract_periods","contract_conversions_count")(p => (p.contract_id,p.aid,p.contract_type,p.name,p.description,p.create_date,p.landing_page_url,p.licensee_id,p.seats_number,p.is_hard_seats_limit_type,p.rid,p.schedule_id,p.contract_is_active,p.contract_periods,p.contract_conversions_count))

final case class ContractPeriod(
  period_id: String,
  name: String,
  sell_date: Long,
  begin_date: Long,
  end_date: Long,
  status: SchedulePeriodStatus,
)
object ContractPeriod:
  implicit val de: Decoder[ContractPeriod] = Decoder.forProduct6[ContractPeriod,String,String,Long,Long,Long,SchedulePeriodStatus]("period_id","name","sell_date","begin_date","end_date","status")(ContractPeriod.apply)
  implicit val en: Encoder[ContractPeriod] = Encoder.forProduct6[ContractPeriod,String,String,Long,Long,Long,SchedulePeriodStatus]("period_id","name","sell_date","begin_date","end_date","status")(p => (p.period_id,p.name,p.sell_date,p.begin_date,p.end_date,p.status))

sealed trait SchedulePeriodStatus:
  def value: String

object SchedulePeriodStatus:
  implicit val en: Encoder[SchedulePeriodStatus] = Encoder[String].contramap[SchedulePeriodStatus](_.value)
  case object Activated extends SchedulePeriodStatus:
    def value: String = "ACTIVATED"

  case object Active extends SchedulePeriodStatus:
    def value: String = "ACTIVE"

  case object Inactive extends SchedulePeriodStatus:
    def value: String = "INACTIVE"

  case object Ended extends SchedulePeriodStatus:
    def value: String = "ENDED"

